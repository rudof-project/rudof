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
