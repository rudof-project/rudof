# Use of AI

You may use AI assistants to work on rudof. Many of us do. What follows is not a restriction on
the tools you use, it is a statement about who is accountable for the result.

## The principle

**You are the author of your contribution, and you are responsible for all of it.**

A tool that helped you write code is not a contributor, in the same way that a compiler, an IDE or
a search engine is not a contributor. The person who opens the pull request is the person who
vouches for the code, answers questions about it in review, and is credited for it.

## What this means in practice

**No AI co-authorship trailers.** Commits must not carry `Co-Authored-By:` trailers naming an AI
assistant (`Claude`, `Copilot`, `Cursor`, `Devin`, or any other). Several tools add these
automatically; remove them before pushing. Trailers crediting a *person* who genuinely co-authored
the change are welcome and unaffected.

This is a deliberate choice, and it is about accountability rather than credit. A commit signed
only by you states without ambiguity that you stand behind it. A commit co-signed by a tool
suggests a share of the responsibility that a tool cannot hold.

**You must be able to explain your pull request.** Every line, in review: why it is there, what it
does, and why it is correct. "The model wrote it that way" is not an answer a reviewer can act on.
If you cannot explain a piece of generated code, do not submit it.

**No unsolicited bulk contributions.** Pull requests generated at scale, against issues the author
has not understood, cost reviewers more than they are worth. Maintainers will close them. One
considered pull request is worth more than ten generated ones.

**Reviews follow the same rule.** You may use AI to help review a pull request, but the comment you
post is yours: you have read it, you agree with it, and you can defend it.

## Disclosure

You are not required to disclose that you used an assistant, and there is no field asking. That
follows from the principle: if the contribution is entirely yours to account for, how you wrote it
is your business. What is not optional is being able to stand behind it.
