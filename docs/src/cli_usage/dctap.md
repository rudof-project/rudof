# DCTAP

`rudof` supports [DCTAP (Dublin Core Tabular Application Profiles)](https://www.dublincore.org/specifications/dctap/).

DCTAP is a simple tabular template that can be used to describe application profiles.

As an example, the following example represents 2 shapes: `Person` and `Company`.
Each row in DCTAP can contain a statement template that describes a property of that shape and some constraints.
For example, the shape `Person` has 4 templates about the properties `name`, `birthDate`, `knows` and `gender`.

| shapeId  | propertyId | valueShape | valueDatatype | valueConstraint | mandatory | repeatable |
|-------|------|-------|------|-------|---|---|
| Person | name |  | xsd:string |  |  true | false |
|  | birthDate |  | xsd:date |  |  false | false |
| | knows | Person | | | false | true |
| | gender | | | male female other | false | false |
| Company| founder | Person | |  | false | false |

`rudof` supports DCTAP files in both CSV format and Excel spreadsheets.

Assuming the previous table is represented in a `person.csv` file as this one:

```csv
shapeId,propertyId,valueShape, valueDatatype, valueConstraint, mandatory, repeatable
Person,name,,xsd:string,,true,false
,birthDate,,xsd:date,,false,false
,gender,,,male female other,false,false
,knows, Person,,,false,true
,worksFor, Company,,,false,true
,,,,
Company,founder, Person,,,false,false
```

The following command can process the file and represent the shapes that it contains:

```sh
❯ rudof dctap -s user.csv
Shape(Person)  
 name xsd:string 
 birthDate xsd:date ?
 gender [male female other]?
 knows @Person *
 worksFor @Company *
Shape(Company)  
 founder @Person ?
```

Apart of CSV, `rudof` can also directly process files in spreadsheet formats like `xlsx`, assuming the previous table is stored in a file `user.xlsx`, it can be processed as:

```sh
❯ rudof dctap -s user.xlsx -f xlsx
```

## DCTAP command

The general format of the DCTAP subcommand is:

```sh
❯ rudof dctap --help
Show information and process DCTAP files

Usage: rudof dctap [OPTIONS] --source-file <DCTap source file>

Options:
  -s, --source-file <DCTap source file>
          
  -f, --format <DCTap file format>
          [default: csv] [possible values: csv, xlsx, xlsb, xlsm, xls]
  -r, --result-format <Ouput results format>
          [default: internal] [possible values: internal, json]
  -c, --config-file <Config file name>
          Config file path, if unset it assumes default config
  -o, --output-file <Output file name, default = terminal>
          
      --force-overwrite
          
  -h, --help
          Print help
```

## DCTAP Configuration file

The parameter `--config-file` (`-c` in short form) can be used to pass a configuration file in [TOML](https://toml.io/) format.

The fields that it can contain are:

- delimiter: field delimiter to use when parsing CSV. The default value is `,`. Another typical value is `;`.
- quote: The quote character to use when parsing CSV. The default is `"`. It can be used to indicate single quotes instead of double quotes.
- flexible: a boolean value that indicates if the number of fields in records is allowed to change or not. When disabled, parsing CSV data will return an error if a record is found with a number of fields different from the number of fields in a previous record. It is enabled by default.
- picklist_delimiter: character that is used to separate values in a picklist cell. The default value is a bar `|`.

The following fields are experimental and may be changed:

- property_placeholders: Table that can be used to generate values for some keys. When the processor finds a cell with some of those keys, it generates a value according to the placeholder resolver indicated.
At this moment, `rudof` supports the placeholder resolver `Stem` which means that it will replace the key by the corresponding stem value.
For example, if the property placeholder has the entry `x` with the placeholder resolver of type `Stem` and the value `stem = "Pending"`,
when a cell contains `x:User`, the generated value will be: `pending:User`.

- empty_property_placeholder: Indicates how to generate a value for a row whose property ID is empty.
The value is a placeholder resolver similar to the values in `property_placeholders`.

The following TOML file can be an example:

```toml
[dctap]
delimiter = ","
picklist_delimiter = " "

[property_placeholders.x.Stem]
stem = "pending"
        
[empty_property_placeholder.Stem]
stem = "empty"
```
