import inspect

import pyrudof

classes = sorted(name for name, obj in inspect.getmembers(pyrudof, inspect.isclass))

print("PYRUDOF_CLASSES_OK")
print(f"Contains Rudof: {'Rudof' in classes}")
print(f"Contains RudofConfig: {'RudofConfig' in classes}")
