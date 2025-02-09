import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import seaborn as sns

def show_world():
    df_world = pd.read_csv("world.csv");
    allies = np.array((df_world["ally_x"], df_world["ally_y"]))
    enemies = np.array((df_world["enemy_x"], df_world["enemy_y"]))
    plt.scatter(*allies, c="blue")
    plt.scatter(*enemies, c="red")
    plt.grid()
    plt.title("World setup, allies are blue, pass starts from (0.3, -1.4)")


def show_graph():
    bounds_df = pd.read_csv("bounds.csv")
    x_step, y_step, i_x, i_y = bounds_df.iloc[0]
    df = pd.read_csv("data.csv")
    x = df["x"]
    y = df["y"]
    graph = df["score"].to_numpy();
    print(f"df: {df['score'][4501 + 1]} | np: {df['score'].to_numpy()[4501 + 1]}");
    #graph = graph.reshape((
    #    np.ceil((x.max() - x.min()) / x_step).astype(np.int64),
    #    np.ceil((y.max() - y.min()) / y_step).astype(np.int64),
    #))

    # TODO: temp
    #plt.scatter(df["x"].to_numpy(), df["y"].to_numpy(), c=graph, s=1, cmap="hot")
    #plt.colorbar();
    #plt.show();
    #exit(0)

    print(f"Min : {np.min(graph)}, Max : {np.max(graph)}")
    
    pivotted = df.pivot(columns="x", index="y", values="score")
    
    fig, axes = plt.subplots(2, 2)
    fig.set_size_inches(16, 10)
    ax0 = axes[0, 0]
    sns.heatmap(pivotted, cmap="viridis", ax=ax0)
    print(f"xmin: {x.min()}, xmax: {x.max()}")
    
    # overwrite ticks to make them beautiful
    ax0.set_xticks(
        np.linspace(x.min(), int(i_x), 10),
        [np.round(x, 2) for x in np.linspace(x.min(), x.max(), 10)]
    )
    ax0.set_yticks(
        np.linspace(y.min(), int(i_y), 10),
        [np.round(y, 2) for y in np.linspace(y.min(), y.max(), 10)]
    )
    ax0.invert_yaxis()
    #plt.gca().invert_yaxis() 
    
    plt.title("Opponents blocking trajectory perpendicularly malus")
    #plt.title("Closest ally bonus")
    #plt.title("Opponents collidng with pass trajectory malus")
    #plt.title("Progress on field bonus")
    #plt.title("Sum of all pass condition scores, with equal weight")
    plt.savefig("block.png")

if __name__ == "__main__":
    #plt.xlim(-4.5, 4.5)
    #plt.ylim(-3., 3.)
    #plt.gcf().set_size_inches(16, 9)
    #show_world()
    show_graph()
    plt.show()
