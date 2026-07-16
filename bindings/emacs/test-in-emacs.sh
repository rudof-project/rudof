EMACS="${FOO:=emacs}" # Mac: /Applications/Emacs.app/Contents/MacOS/Emacs
DYLIB="${DYLIB:=../../target/release/libemacs_rudof.so}" # Mac: s/.so/.dylib/
$EMACS --batch --eval "
  (progn
    (module-load \"$DYLIB\")
    (let ((rudof (rudof-new)))
      (rudof-read-shex
       rudof
       \"PREFIX xsd: <http://www.w3.org/2001/XMLSchema#> <http://example.org/PersonShape> { <http://example.org/age> xsd:integer }\"
       \"shexc\" nil)
      (rudof-read-data
       rudof \"<http://example.org/alice> <http://example.org/age> 30 .\" \"turtle\" nil)
      (rudof-read-shapemap
       rudof \"<http://example.org/alice>@<http://example.org/PersonShape>\" nil nil nil)
      (message \"validate: %S\" (rudof-validate-shex rudof))
      (message \"error path (bad format): %S\"
        (condition-case err (rudof-read-data rudof \"x\" \"bogus\" nil) (error err)))
      (message \"error path (validate before anything loaded): %S\"
        (condition-case err (rudof-validate-shex (rudof-new)) (error err)))))
  "
