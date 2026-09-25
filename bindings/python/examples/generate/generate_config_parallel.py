"""Configure and read the parallel generation settings."""

from pyrudof import GeneratorConfig


def main() -> None:
    config = GeneratorConfig()

    config.set_compress(True)
    config.set_write_stats(True)
    config.set_parallel_writing(True)
    config.set_parallel_file_count(2)
    config.set_worker_threads(2)
    config.set_batch_size(16)
    config.set_parallel_shapes(True)
    config.set_parallel_fields(True)

    print(f"compress: {config.get_compress()}")
    print(f"write_stats: {config.get_write_stats()}")
    print(f"parallel_writing: {config.get_parallel_writing()}")
    print(f"parallel_file_count: {config.get_parallel_file_count()}")
    print(f"worker_threads: {config.get_worker_threads()}")
    print(f"batch_size: {config.get_batch_size()}")
    print(f"parallel_shapes: {config.get_parallel_shapes()}")
    print(f"parallel_fields: {config.get_parallel_fields()}")


if __name__ == "__main__":
    main()
