import json

import plotly.graph_objects as go


def plot(durations):
    fig = go.Figure()
    for arch, d in durations.items():
        fig.add_trace(go.Scatter(x=list(range(1, len(d) + 1)), y=d,
                                 mode='lines+markers',
                                 name=arch))

    fig.update_layout(title='Cold start - AWS provided container and Distroless comparison',
                      xaxis_title='Execution',
                      yaxis_title='Duration (milliseconds)',
                      xaxis_tickformat=',d',
                      xaxis=dict(showgrid=True),
                      yaxis=dict(showgrid=True))

    fig.show()


if __name__ == '__main__':
    with open('startup-rs-distroless.json') as rs_distroless, open('startup-rs.json') as rs:
        durations = json.load(rs_distroless)
        # durations = json.load(rs)
        # durations.update(json.load(rs))
        plot(durations)
