from typing import Any

class Rudof:
    def __init__(self, config: dict[str, Any] | None = None) -> None: ...

    def compare_schemas_str(
        self,
        schema1: str,
        mode1: "CompareSchemaMode",
        format1: "CompareSchemaFormat",
        label1: str | None,
        base1: str | None,
        schema2: str,
        mode2: "CompareSchemaMode",
        format2: "CompareSchemaFormat",
        label2: str | None,
        base2: str | None,
    ) -> "ShaCo": ...