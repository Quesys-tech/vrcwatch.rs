import polars as pl
from pathlib import Path
from matplotlib import pyplot as plt
import numpy as np

if __name__ == "__main__":
    # Create a DataFrame with some sample data
    project_dir = Path(__file__).parent
    df = pl.read_ndjson(project_dir / "log.json")
    # convert the timestamp to a datetime object
    df = df.with_columns(pl.col("timestamp").str.to_datetime(time_unit="us",format="%+"))
    start_time = df["timestamp"].min()  # Get the start time
    # add a new column with the elapsed time in microseconds
    df = df.with_columns(
        ((pl.col("timestamp") - start_time).cast(pl.Float64) / 1e6).alias(
            "elapsed_time"
        )
    )

    # add a new column with the subsecond part of the timestamp
    df = df.with_columns(
        (pl.col("timestamp").dt.microsecond() / 1_000_000).alias("jitter")
    )
    # convert the timestamp to a datetime object
    df_awake = df.filter((pl.col("fields").struct.field("message") == "Awake"))
    df_tick_watch = df.filter(pl.col("fields").struct.field("message") == "Tick watch")

    print(df_awake)
    fig, axes = plt.subplots(2, 2, figsize=(12, 8))
    # Plot the elapsed time for each awake event
    axes[0, 0].plot(
        df_awake["elapsed_time"].to_numpy(),
        df_awake["jitter"].to_numpy(),
        color="C0",
    )
    axes[0, 0].set_xlabel("Elapsed time (s)")
    axes[0, 0].set_ylabel("Jitter (s)")
    axes[0, 0].set_title("awake jitter")
    axes[0, 1].hist(
        df_awake["jitter"].to_numpy(), bins=100, density=True, color="C0"
    )
    axes[0, 1].set_xlabel("Jitter (s)")
    axes[0, 1].set_ylabel("Density")
    axes[0, 1].set_title("awake jitter histogram")

    # Plot the elapsed time for each tick watch event
    axes[1, 0].plot(
        df_tick_watch["elapsed_time"].to_numpy(),
        df_tick_watch["jitter"].to_numpy(),
        color="C1",
    )
    axes[1, 0].set_xlabel("Elapsed time (s)")
    axes[1, 0].set_ylabel("Jitter (s)")
    axes[1, 0].set_title("tick watch jitter")
    axes[1, 1].hist(
        df_tick_watch["jitter"].to_numpy(), bins=100, density=True, color="C1"
    )
    axes[1, 1].set_xlabel("Jitter (s)")
    axes[1, 1].set_ylabel("Density")
    axes[1, 1].set_title("tick watch jitter histogram")

    fig.tight_layout()
    fig.savefig(project_dir / "jitter.png")
