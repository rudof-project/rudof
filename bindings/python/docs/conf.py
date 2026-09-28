"""Sphinx configuration for the ``pyrudof`` documentation."""

import ast
import datetime
from pathlib import Path
from typing import Any

import pyrudof

# -- Project information -----------------------------------------------------

project = "pyrudof"
copyright = f"{datetime.date.today().year}, pyrudof contributors"
author = "pyrudof contributors"
version = pyrudof.__version__
release = pyrudof.__version__

# -- General configuration ---------------------------------------------------

extensions = [
    "sphinx.ext.autodoc",
    "sphinx.ext.autosummary",
    "sphinx.ext.doctest",
    "sphinx.ext.intersphinx",
    "sphinx.ext.napoleon",
    "sphinx.ext.todo",
    "sphinx.ext.coverage",
]

templates_path = ["_templates"]
exclude_patterns = ["build", "Thumbs.db", ".DS_Store", "_build", "**.ipynb_checkpoints"]

# Unresolved cross-references fail the build rather than silently rendering as plain text.
nitpicky = True
nitpick_ignore = [
    ("py:class", "module"),
    ("py:class", "pyrudof._pyrudof.PyRudof"),
]

# -- Autodoc configuration ---------------------------------------------------

autodoc_member_order = "bysource"
autodoc_typehints = "both"
autodoc_typehints_description_target = "documented"
autodoc_class_signature = "separated"
autodoc_default_options = {
    "members": True,
    "member-order": "bysource",
    "special-members": "__init__",
    "undoc-members": True,
    "exclude-members": "__weakref__",
}

# -- Napoleon configuration (Google/NumPy docstring support) -----------------

napoleon_google_docstring = True
napoleon_numpy_docstring = True
napoleon_include_init_with_doc = True
napoleon_include_private_with_doc = False
napoleon_include_special_with_doc = True
napoleon_use_admonition_for_examples = True
napoleon_use_admonition_for_notes = True
napoleon_use_admonition_for_references = False
napoleon_use_ivar = False
napoleon_use_param = True
napoleon_use_rtype = True
napoleon_preprocess_types = True
napoleon_attr_annotations = True

napoleon_type_aliases = {
    "str | os.PathLike": ":py:class:`str` | :py:class:`os.PathLike`",
    "list[str]": ":py:class:`list`\\[:py:class:`str`]",
}

# -- Autosummary configuration -----------------------------------------------

autosummary_generate = True
autosummary_imported_members = False

# -- Options for HTML output -------------------------------------------------

html_permalinks_icon = "<span>#</span>"
html_theme = "sphinxawesome_theme"
html_static_path = ["_static"]
_LOGO = (
    "https://raw.githubusercontent.com/rudof-project/rudof/refs/heads/master"
    "/docs/src/assets/corp/logo.svg"
)
html_logo = _LOGO
html_favicon = _LOGO
_GITHUB_MARK = (
    '<svg viewBox="0 0 24 24" aria-hidden="true" height="26px">'
    '<path fill="currentColor" d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205'
    "11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61"
    "C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236"
    "1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466"
    "-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005"
    "-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23"
    "3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625"
    "-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57"
    'C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"/></svg>'
)

html_theme_options = {
    "show_prev_next": True,
    "show_scrolltop": True,
    "main_nav_links": {"Examples": "examples", "API reference": "library"},
    "extra_header_link_icons": {
        "rudof on GitHub": {
            "link": "https://github.com/rudof-project/rudof",
            "icon": _GITHUB_MARK,
        },
    },
}
html_baseurl = "https://pyrudof.readthedocs.io/en/stable/"
html_context = {
    "display_github": True,
    "github_user": "rudof-project",
    "github_repo": "rudof",
    "github_version": "master",
    "conf_py_path": "/bindings/python/docs/",
}

html_show_sourcelink = False
html_copy_source = False
html_css_files = ["custom.css"]

# -- Options for intersphinx -------------------------------------------------

intersphinx_mapping = {
    "python": ("https://docs.python.org/3", None),
}

# -- Options for todo extension ----------------------------------------------

todo_include_todos = True

# -- Options for linkcheck ---------------------------------------------------

linkcheck_ignore = [
    r"http://localhost.*",
    r"https://example\.org/.*",
]

# -- Additional configuration ------------------------------------------------

pygments_style = "sphinx"
pygments_dark_style = "monokai"


# -- Enum variant docstrings, recovered from the type stub -------------------
#
# Every format enum is a pyo3 unit enum, and its variants are plain instances of the enum
# class. Two things follow, and both need fixing here:
#
# 1. A variant has no ``__doc__`` of its own.
# 2. Without a Python source file to analyse, ``autodoc_member_order = "bysource"`` has no
#    source order to use and the variants come out alphabetised.
#
# The per-variant text written in ``formats/*.rs`` does survive into the generated
# ``_pyrudof/__init__.pyi`` as a PEP 258 attribute docstring, in declaration order. So
# parse the stub and feed autodoc both the text and the order, via a documenter that
# outranks the stock one for exactly these members.

_STUB = Path(__file__).resolve().parent.parent / "python" / "pyrudof" / "_pyrudof" / "__init__.pyi"


def _parse_stub() -> tuple[dict[tuple[str, str], str], dict[tuple[str, str], int]]:
    """Return ``(docstrings, declaration order)`` keyed by ``(class, attribute)``."""
    if not _STUB.is_file():
        return {}, {}

    module = ast.parse(_STUB.read_text(encoding="utf-8"))
    docs: dict[tuple[str, str], str] = {}
    order: dict[tuple[str, str], int] = {}

    for cls in module.body:
        if not isinstance(cls, ast.ClassDef):
            continue
        # An attribute docstring is a bare string expression *following* the assignment.
        pending: str | None = None
        for stmt in cls.body:
            if isinstance(stmt, (ast.Assign, ast.AnnAssign)):
                targets = stmt.targets if isinstance(stmt, ast.Assign) else [stmt.target]
                names = [t.id for t in targets if isinstance(t, ast.Name)]
                pending = names[0] if len(names) == 1 else None
                if pending is not None:
                    order.setdefault((cls.name, pending), len(order))
            elif (
                pending is not None
                and isinstance(stmt, ast.Expr)
                and isinstance(stmt.value, ast.Constant)
                and isinstance(stmt.value.value, str)
            ):
                docs[(cls.name, pending)] = stmt.value.value.strip()
                pending = None
            else:
                pending = None

    return docs, order


_VARIANT_DOCS, _VARIANT_ORDER = _parse_stub()


def _variant_key(parent: Any, membername: str) -> tuple[str, str] | None:
    """The ``_VARIANT_DOCS`` key for a member, or ``None`` if it is not a variant."""
    cls_name = getattr(parent, "__name__", None)
    if cls_name is None:
        return None
    key = (cls_name, membername)
    return key if key in _VARIANT_DOCS else None


def _install_documenter(app: Any) -> None:
    from sphinx.ext.autodoc import AttributeDocumenter

    class EnumVariantDocumenter(AttributeDocumenter):
        """Documents a pyo3 unit-enum variant using the docstring from the type stub."""

        objtype = "pyenumvariant"
        directivetype = "attribute"
        priority = AttributeDocumenter.priority + 10

        @classmethod
        def can_document_member(
            cls, member: Any, membername: str, isattr: bool, parent: Any
        ) -> bool:
            return _variant_key(getattr(parent, "object", None), membername) is not None

        def get_doc(self) -> list[list[str]] | None:
            key = _variant_key(self.parent, self.objpath[-1])
            if key is None:
                return super().get_doc()
            return [_VARIANT_DOCS[key].splitlines()]

    app.add_autodocumenter(EnumVariantDocumenter)


def _install_ordering(app: Any) -> None:
    from sphinx.ext.autodoc import ClassDocumenter

    _stock_sort = ClassDocumenter.sort_members

    def sort_members(self: Any, documenters: list[Any], order: str) -> list[Any]:
        documenters = _stock_sort(self, documenters, order)

        # Permute only the variant entries into declaration order, leaving every other
        # member — methods, classmethods — exactly where the stock sort put it.
        positions = []
        ranked = []
        for i, entry in enumerate(documenters):
            membername = entry[0].name.rsplit(".", 1)[-1].lstrip(":")
            key = _variant_key(getattr(self, "object", None), membername)
            if key is not None:
                positions.append(i)
                ranked.append((_VARIANT_ORDER.get(key, 0), entry))

        for position, (_, entry) in zip(positions, sorted(ranked, key=lambda p: p[0])):
            documenters[position] = entry

        return documenters

    ClassDocumenter.sort_members = sort_members  # type: ignore[method-assign]


def setup(app: Any) -> dict[str, Any]:
    _install_documenter(app)
    _install_ordering(app)
    return {"parallel_read_safe": True, "parallel_write_safe": True}
