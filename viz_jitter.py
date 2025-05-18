import polars as pl
from pathlib import Path
from matplotlib import pyplot as plt

if __name__ == "__main__":
    # Create a DataFrame with some sample data
    project_dir = Path(__file__).parent
    df = pl.read_ndjson(project_dir / "log.json")
    df_ticktack = df.filter(
        (pl.col("level") == "DEBUG")
        & (
            (pl.col("fields").struct.field("message") == "Awake")
            | (pl.col("fields").struct.field("message") == "Tick watch")
        )
    )
    # convert the timestamp column to datetime
    df_ticktack = df_ticktack.with_columns(
        pl.col("timestamp").str.strptime(pl.Datetime, "%Y-%m-%dT%H:%M:%S.%fZ")
    )
    # calculate the sub-second part of the timestamp
    df_ticktack = df_ticktack.with_columns(
         pl.col("timestamp").dt.millisecond().alias("sub_second")
    )
    print(df_ticktack)
    # Create a scatter plot with jitter
    plt.plot(df_ticktack["timestamp"], df_ticktack["sub_second"])
    plt.xlabel("Timestamp")
    plt.ylabel("Jitter (ms)")
    plt.show()