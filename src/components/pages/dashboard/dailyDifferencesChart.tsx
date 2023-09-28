import { Chart, Colors, Legend, Title, Tooltip } from "chart.js"
import { Line, PolarArea } from "solid-chartjs"
import { For, Show, onMount } from "solid-js"
import { createStore } from "solid-js/store";


type DailyDifference = {
    id: String,
    volume: number,
    mass: number,
    date: string,
}
type chartProps = {
    data: Array<DailyDifference>
};

export default function DailyDifferencesChart(props: chartProps){
    onMount(() => {
        Chart.register(Title, Tooltip, Legend, Colors)
    })

    const date = new Date();
    let lastDay = new Date(date.getFullYear(), date.getMonth() + 1, 0);
    const month_int = date.getMonth() + 1;
    const month_str = month_int > 10 ? `${month_int}` : `0${month_int}`
    let dateTemplate = `${date.getFullYear()}-${month_str}-`
    let dates = [];

    let hfo_masses = [];
    let hfo_volumes = [];
    let mgo_masses = [];
    let mgo_volumes = [];
    let differences = {};

    for (let index=1; index <= lastDay.getDate(); index++){
        dates.push(
            dateTemplate + index
        )
    }
    
    
    const grapData = () => {
        dates.forEach(
            date => {
                differences[date] = {}
                differences[date]["hfo_volume"] = 0
                differences[date]["hfo_mass"] = 0
                differences[date]["mgo_volume"] = 0
                differences[date]["mgo_mass"] = 0
            }
        );
        props.data.forEach(
            difference => {
                differences[difference.date]["hfo_volume"] = difference.hfo_volume
                differences[difference.date]["hfo_mass"] = difference.hfo_mass
                differences[difference.date]["mgo_volume"] = difference.mgo_volume
                differences[difference.date]["mgo_mass"] = difference.mgo_mass
        }
        );

        dates.forEach(
            date => {
                hfo_volumes.push(differences[date]["hfo_volume"])
                hfo_masses.push(differences[date]["hfo_mass"])
                mgo_volumes.push(differences[date]["mgo_volume"])
                mgo_masses.push(differences[date]["mgo_mass"])
            }
        );
    }

    const mockData = {
        labels: dates,
        datasets: [
            {
                label: `HFO Volume`,
                data: [],
            },
            {
                label: `HFO Mass`,
                data: [],
            },
            {
                label: `MGO Volume`,
                data: [],
            },
            {
                label: `MGO Mass`,
                data: [],
            },
        ],
    }
    const data = {
            labels: dates,
            datasets: [
                {
                    label: `HFO Volume`,
                    data: hfo_volumes,
                    pointStyle: "circle",
                    pointRadius: 10,
                },
                {
                    label: `HFO Mass`,
                    data: hfo_masses,
                    pointStyle: "triangle",
                    pointRadius: 10,
                },
                {
                    label: `MGO Volume`,
                    data: mgo_volumes,
                    pointStyle: "circle",
                    pointRadius: 10,
                },
                {
                    label: `MGO Mass`,
                    data: mgo_masses,
                    pointStyle: "triangle",
                    pointRadius: 10,
                },
            ],
        }
    const chartOptions = {
        responsive: true,
        maintainAspectRatio: false,
    }

    return (
        <div class="chart">
            <Show when={props.data} 
            fallback={
                <Line data={mockData} options={chartOptions} width={500} height={500} />
            }
            >
                {grapData()}
                <Line data={data} options={chartOptions} width={500} height={500} />
            </Show>
        </div>
    )
}