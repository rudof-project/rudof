EMACS=/Applications/Emacs.app/Contents/MacOS/Emacs
DYLIB=/Users/eric/checkouts/rudof-project/rudof/target/release/librudof_emacs.dylib
$EMACS --batch --eval "
  (progn
    (module-load \"$DYLIB\")
    (let ((rudof (rudof-emacs-new)))
      (rudof-emacs-read-shex
       rudof
       \"PREFIX xsd: <http://www.w3.org/2001/XMLSchema#> <http://example.org/PersonShape> { <http://example.org/age> xsd:integer }\"
       \"shexc\" nil)
      (rudof-emacs-read-data
       rudof \"<http://example.org/alice> <http://example.org/age> 30 .\" \"turtle\" nil)
      (rudof-emacs-read-shapemap
       rudof \"<http://example.org/alice>@<http://example.org/PersonShape>\" nil nil nil)
      (message \"validate: %S\" (rudof-emacs-validate-shex rudof))
      (message \"error path (bad format): %S\"
        (condition-case err (rudof-emacs-read-data rudof \"x\" \"bogus\" nil) (error err)))
      (message \"error path (validate before anything loaded): %S\"
        (condition-case err (rudof-emacs-validate-shex (rudof-emacs-new)) (error err)))))
  "
