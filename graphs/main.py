import json

import matplotlib.pyplot as plt


def plot(durations):
    x = [i + 1 for i, _ in enumerate(durations['docker_arm64'])]
    for key, duration in durations.items():
        plt.plot(x, [d / 1000 for d in duration], label=key, marker='o')

    plt.ylim([20, 30])

    # naming the x axis
    plt.xlabel('Run')
    # naming the y axis
    plt.ylabel('Time (seconds)')

    # giving a title to my graph
    plt.title('Lambda Execution Time')

    plt.legend()

    # function to show the plot
    plt.show()


if __name__ == '__main__':
    with open('durations.json') as f:
        plot(json.load(f))
        # print(list(enumerate(json.load(f)['docker_arm64'])))
