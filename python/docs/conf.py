import datetime
import sys
from pathlib import Path

import pyrudof

sys.path.insert(0, str(Path(__file__).parent.parent.absolute()))

# -- Project information -----------------------------------------------------

project = "pyrudof"
copyright = f"{datetime.date.today().year}, pyrudof contributors"
author = pyrudof.__author__
version = pyrudof.__version__
release = pyrudof.__version__

# -- General configuration ---------------------------------------------------

extensions = [
    'sphinx.ext.autodoc',
    'sphinx.ext.autosummary',
    'sphinx.ext.doctest',
    'sphinx.ext.intersphinx',
    'sphinx.ext.napoleon',
    'sphinx.ext.viewcode',
    'sphinx.ext.todo',
    'sphinx.ext.coverage',
    #'sphinx_copybutton',
]

templates_path = ['_templates']
exclude_patterns = ["build", "Thumbs.db", ".DS_Store", "_build", "**.ipynb_checkpoints"]

# -- Autodoc configuration --------------------------------------------------

autodoc_member_order = 'bysource'
autodoc_typehints = 'both'
autodoc_typehints_description_target = 'documented'
autodoc_class_signature = 'separated'
autodoc_default_options = {
    'members': True,
    'member-order': 'bysource',
    'special-members': '__init__',
    'undoc-members': True,
    'exclude-members': '__weakref__'
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
napoleon_type_aliases = None
napoleon_attr_annotations = True

# -- Autosummary configuration -----------------------------------------------

autosummary_generate = True
autosummary_imported_members = False

# -- Options for HTML output -------------------------------------------------

html_permalinks_icon = '<span>#</span>'
html_theme = "sphinxawesome_theme"
html_static_path = ['_static']
html_logo = "https://raw.githubusercontent.com/rudof-project/rudof/refs/heads/master/docs/src/assets/corp/logo.svg"
html_favicon = "https://raw.githubusercontent.com/rudof-project/rudof/refs/heads/master/docs/src/assets/corp/logo.svg"
html_theme_options = {
    "body_max_width": None,
    "show_prev_next": True,
    "show_scrolltop": True,
}
html_baseurl = "https://pyrudof.readthedocs.io/en/stable/"
html_context = {
    "display_github": True,
    "github_user": "rudof-project",
    "github_repo": "rudof",
    "github_version": "master",
    "conf_py_path": "/python/docs/",
}

html_show_sourcelink = True
html_copy_source = False

# -- Options for intersphinx -------------------------------------------------

intersphinx_mapping = {
    "python": ("https://docs.python.org/3", None),
    "rdflib": ("https://rdflib.readthedocs.io/en/stable/", None),
    "requests": ("https://requests.readthedocs.io/en/latest/", None),
}

# -- Options for todo extension ----------------------------------------------

todo_include_todos = True

# -- Options for linkcheck ---------------------------------------------------

linkcheck_ignore = [
    r'http://localhost.*',
    r'https://example\.org/.*',
]

# -- Additional configuration ------------------------------------------------

# Syntax highlighting
pygments_style = 'sphinx'
pygments_dark_style = 'monokai'

# -- Custom CSS (_static/custom.css) ----------------------------------
html_css_files = [
    'custom.css',
]