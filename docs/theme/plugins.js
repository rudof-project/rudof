function darkModeToggle() {
    var html = document.documentElement;
    var themeToggleButton = document.getElementById("theme-toggle");
    var themePopup = document.getElementById("theme-list");
    var themePopupButtons = themePopup.querySelectorAll("button");

    function setTheme(theme) {
        html.setAttribute("data-theme", theme);
        html.setAttribute("data-color-scheme", theme);
        html.className = theme;
        localStorage.setItem("mdbook-theme", theme);

        // Force a repaint to ensure the changes take effect in the client immediately
        document.body.style.display = "none";
        document.body.offsetHeight;
        document.body.style.display = "";
    }

    themeToggleButton.addEventListener("click", function (event) {
        event.preventDefault();
        themePopup.style.display =
        themePopup.style.display === "block" ? "none" : "block";
    });

    themePopupButtons.forEach(function (button) {
        button.addEventListener("click", function () {
        setTheme(this.id);
        themePopup.style.display = "none";
        });
    });

    document.addEventListener("click", function (event) {
        if (
        !themePopup.contains(event.target) &&
        !themeToggleButton.contains(event.target)
        ) {
        themePopup.style.display = "none";
        }
    });

    // Set initial theme
    var currentTheme = localStorage.getItem("mdbook-theme");
    if (currentTheme) {
        setTheme(currentTheme);
    } else {
        // If no theme is set, use the system's preference
        var systemPreference = window.matchMedia("(prefers-color-scheme: dark)")
        .matches
        ? "dark"
        : "light";
        setTheme(systemPreference);
    }

    // Listen for system's preference changes
    const darkModeMediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
    darkModeMediaQuery.addEventListener("change", function (e) {
        if (!localStorage.getItem("mdbook-theme")) {
        setTheme(e.matches ? "dark" : "light");
        }
    });
}