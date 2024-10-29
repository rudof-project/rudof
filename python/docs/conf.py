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
    'sphinx.ext.doctest',
    'sphinx.ext.intersphinx',
    'sphinx_rtd_theme'
]

exclude_patterns = ["build", "Thumbs.db", ".DS_Store"]

# -- Options for HTML output -------------------------------------------------

html_theme = "sphinx_rtd_theme"
html_static_path = []
html_logo = "https://raw.githubusercontent.com/rudof-project/rudof-project.github.io/refs/heads/main/images/rudof_logo3.svg"
html_favicon = "https://raw.githubusercontent.com/rudof-project/rudof-project.github.io/refs/heads/main/images/rudof_logo3.svg"
html_theme_options = {"body_max_width": None}
html_baseurl = "https://pyrudof.readthedocs.io/en/stable/"

# -- Options for doctests -------------------------------------------------

doctest_global_setup = "from pyrudof import *"

# -- Options for intersphinx -------------------------------------------------

intersphinx_mapping = {"python": ("https://docs.python.org/3", None)}