#!/usr/bin/env python3
"""Move issues across the project board.

The board columns are the `Status` field of the project; the issue number comes from
the `Refs #N` lines of a pull request body, because the policy forbids closing
keywords and GitHub therefore creates no link of its own.

Subcommands, one per event:

    issue       an issue was opened, reopened or closed
    pull        a pull request was opened, reopened, marked ready or closed
    release     a release was published: everything in `Merged` is now released

Needs `GH_TOKEN` with read and write access to the organization projects.
"""

import json
import os
import re
import subprocess
import sys

OWNER = os.environ.get("PROJECT_OWNER", "rudof-project")
NUMBER = int(os.environ.get("PROJECT_NUMBER", "3"))
REPO = os.environ.get("GITHUB_REPOSITORY", "rudof-project/rudof")
PENDING_RELEASE = "status/pending-release"
REFS = re.compile(r"\brefs?\b[:\s]*#(\d+)", re.IGNORECASE)


def gh(*args, **kwargs):
    out = subprocess.run(["gh", *args], capture_output=True, text=True, **kwargs)
    if out.returncode != 0:
        sys.exit(f"gh {' '.join(args[:3])} failed: {out.stderr.strip()[:500]}")
    return out.stdout


def graphql(query, **variables):
    args = ["api", "graphql", "-f", f"query={query}"]
    for key, value in variables.items():
        # -F infers the type, which is what Int! variables need; -f keeps strings as strings.
        args += ["-F" if isinstance(value, int) else "-f", f"{key}={value}"]
    data = json.loads(gh(*args))
    if "errors" in data:
        sys.exit("graphql: " + json.dumps(data["errors"])[:500])
    return data["data"]


PROJECT = """
query($owner: String!, $number: Int!) {
  organization(login: $owner) {
    projectV2(number: $number) {
      id
      field(name: "Status") {
        ... on ProjectV2SingleSelectField { id options { id name } }
      }
    }
  }
}"""


def project():
    if not hasattr(project, "cached"):
        data = graphql(PROJECT, owner=OWNER, number=NUMBER)["organization"]["projectV2"]
        project.cached = (
            data["id"],
            data["field"]["id"],
            {o["name"]: o["id"] for o in data["field"]["options"]},
        )
    return project.cached


ISSUE = """
query($owner: String!, $name: String!, $number: Int!) {
  repository(owner: $owner, name: $name) {
    issue(number: $number) {
      id
      state
      labels(first: 30) { nodes { name } }
      projectItems(first: 10) { nodes { id project { number } } }
    }
  }
}"""

ADD_ITEM = """
mutation($project: ID!, $content: ID!) {
  addProjectV2ItemById(input: {projectId: $project, contentId: $content}) {
    item { id }
  }
}"""

SET_STATUS = """
mutation($project: ID!, $item: ID!, $field: ID!, $option: String!) {
  updateProjectV2ItemFieldValue(input: {projectId: $project, itemId: $item,
      fieldId: $field, value: {singleSelectOptionId: $option}}) {
    projectV2Item { id }
  }
}"""


def issue(number):
    owner, name = REPO.split("/")
    data = graphql(ISSUE, owner=owner, name=name, number=number)
    return data["repository"]["issue"]


def move(number, column, node_id=None):
    """Put issue `number` in `column`, adding it to the project if it is not there."""
    project_id, field_id, options = project()
    if column not in options:
        sys.exit(f"the project has no column named {column!r}")
    if node_id is None:
        node_id = issue(number)["id"]
    # addProjectV2ItemById returns the existing item when the issue is already on the board.
    item = graphql(ADD_ITEM, project=project_id, content=node_id)
    item_id = item["addProjectV2ItemById"]["item"]["id"]
    graphql(SET_STATUS, project=project_id, item=item_id,
            field=field_id, option=options[column])
    print(f"#{number} -> {column}")


def referenced_issues(body):
    numbers = sorted({int(n) for n in REFS.findall(body or "")})
    if not numbers:
        print("no 'Refs #N' in the pull request body, nothing to move")
    return numbers


def cmd_issue():
    number = int(os.environ["ISSUE_NUMBER"])
    action = os.environ["EVENT_ACTION"]
    if action == "opened":
        move(number, "Triage")
    elif action == "reopened":
        move(number, "Backlog")
    elif action == "closed":
        # Closing as not planned or duplicate is not a release; leave those where they are.
        if os.environ.get("STATE_REASON", "").upper() == "COMPLETED":
            move(number, "Released")
        else:
            print(f"#{number} closed as {os.environ.get('STATE_REASON')}, not moved")


def cmd_pull():
    action = os.environ["EVENT_ACTION"]
    merged = os.environ.get("PR_MERGED") == "true"
    draft = os.environ.get("PR_DRAFT") == "true"
    if action == "closed" and not merged:
        column = "Ready"
    elif action == "closed":
        column = "Merged"
    else:
        column = "In progress" if draft else "In review"
    for number in referenced_issues(os.environ.get("PR_BODY", "")):
        move(number, column)
        if column == "Merged":
            labels = [l["name"] for l in issue(number)["labels"]["nodes"]]
            if PENDING_RELEASE not in labels:
                gh("issue", "edit", str(number), "-R", REPO,
                   "--add-label", PENDING_RELEASE)
                print(f"#{number} labelled {PENDING_RELEASE}")


ITEMS = """
query($project: ID!, $cursor: String) {
  node(id: $project) {
    ... on ProjectV2 {
      items(first: 100, after: $cursor) {
        pageInfo { hasNextPage endCursor }
        nodes {
          id
          fieldValueByName(name: "Status") {
            ... on ProjectV2ItemFieldSingleSelectValue { name }
          }
          content { ... on Issue { number state repository { nameWithOwner } } }
        }
      }
    }
  }
}"""


def merged_items():
    project_id = project()[0]
    cursor, found = None, []
    while True:
        page = graphql(ITEMS, project=project_id, **({"cursor": cursor} if cursor else {})
                       )["node"]["items"]
        for item in page["nodes"]:
            content, status = item["content"], item["fieldValueByName"]
            if not content or not content.get("number"):
                continue
            if content["repository"]["nameWithOwner"] != REPO:
                continue
            if status and status["name"] == "Merged":
                found.append((item["id"], content["number"], content["state"]))
        if not page["pageInfo"]["hasNextPage"]:
            return found
        cursor = page["pageInfo"]["endCursor"]


def cmd_release():
    tag = os.environ.get("RELEASE_TAG", "")
    items = merged_items()
    print(f"{len(items)} issues in Merged at release {tag}")
    for _, number, state in items:
        labels = [l["name"] for l in issue(number)["labels"]["nodes"]]
        if PENDING_RELEASE in labels:
            gh("issue", "edit", str(number), "-R", REPO,
               "--remove-label", PENDING_RELEASE)
        if state == "OPEN":
            gh("issue", "close", str(number), "-R", REPO, "--reason", "completed",
               "--comment", f"Released in [{tag}](https://github.com/{REPO}/releases/tag/{tag}).")
        move(number, "Released")


if __name__ == "__main__":
    {"issue": cmd_issue, "pull": cmd_pull, "release": cmd_release}[sys.argv[1]]()
