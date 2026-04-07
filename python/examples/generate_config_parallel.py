from pyrudof import GeneratorConfig

config = GeneratorConfig()

config.set_compress(True)
config.set_write_stats(True)
config.set_parallel_writing(True)
config.set_parallel_file_count(2)
config.set_worker_threads(2)
config.set_batch_size(16)
config.set_parallel_shapes(True)
config.set_parallel_fields(True)

print("GEN_CONFIG_PARALLEL_OK")
print(f"Compress: {config.get_compress()}")
print(f"Write stats: {config.get_write_stats()}")
print(f"Parallel writing: {config.get_parallel_writing()}")
print(f"Parallel file count: {config.get_parallel_file_count()}")
print(f"Worker threads: {config.get_worker_threads()}")
print(f"Batch size: {config.get_batch_size()}")
print(f"Parallel shapes: {config.get_parallel_shapes()}")
print(f"Parallel fields: {config.get_parallel_fields()}")
