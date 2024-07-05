from plotter_guest import plotter

import os
from os import path

import plotly

import plotly.graph_objs as go
from plotly.graph_objs import _figure, _deprecations, _layout, layout
from plotly.graph_objs.layout import _template

import plotly.package_data.templates

import plotly.validators
from plotly.validators import _data, _layout, _frames

import plotly.offline
import plotly.animation
import plotly.io
import random


class Run:
    def run(self):
        plot = Plot()
        plot.test_plot()


class Plot(plotter.Plotter):
    def plot_temperature(self, idx: list[int], temperatures: list[int]) -> None:
        return

    def plot_humidity(self, idx: list[int], humidities: list[int]) -> None:
        return

    def test_plot(self) -> None:
        random.seed(1)

        N = 100
        x = random.randint(1, N)
        y = random.randint(1, N)
        colors = random.randint(1, N)
        sz = random.randint(1, N) * 30

        fig = go.Figure()
        fig.add_trace(go.Scatter(x=x, y=y, size=sz, color=colors))

        if not path.exists("images"):
            os.mkdir("images")

        fig.to_image(format="png", engine="kaleido")
