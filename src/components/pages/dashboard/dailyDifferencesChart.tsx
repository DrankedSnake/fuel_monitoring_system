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

    let masses = [];
    let volumes = [];
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
                differences[date]["volume"] = 0
                differences[date]["mass"] = 0
            }
        );
        props.data.forEach(
            difference => {
                differences[difference.date]["volume"] = difference.volume
                differences[difference.date]["mass"] = difference.mass
        }
        );

        dates.forEach(
            date => {
                volumes.push(differences[date]["volume"])
                masses.push(differences[date]["mass"])
            }
        );
    }

    const mockData = {
        labels: dates,
        datasets: [
            {
                label: `Volume`,
                data: [],
            },
            {
                label: `Mass`,
                data: [],
            },
        ],
    }
    const data = {
            labels: dates,
            datasets: [
                {
                    label: `Volume`,
                    data: volumes,
                    pointStyle: "circle",
                    pointRadius: 10,
                },
                {
                    label: `Mass`,
                    data: masses,
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