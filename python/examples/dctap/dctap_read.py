from pathlib import Path
from tempfile import TemporaryDirectory

from pyrudof import DCTapFormat, Rudof, RudofConfig

rudof = Rudof(RudofConfig())

csv_text = "shapeId,propertyId\n:Person,:name\n"
rudof.read_dctap(csv_text)

with TemporaryDirectory() as tmpdir:
    csv_path = Path(tmpdir) / "profile.csv"
    csv_path.write_text(csv_text, encoding="utf-8")
    rudof.read_dctap(str(csv_path), DCTapFormat.Csv)
