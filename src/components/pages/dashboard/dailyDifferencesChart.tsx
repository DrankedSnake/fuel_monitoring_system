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

    let dateTemplate = `${date.getFullYear()}/${date.getMonth() + 1}/`
    console.log(dateTemplate)
    let dates = [];

    let masses = [];
    let volumes = [];

    for (let index=1; index <= lastDay.getDate(); index++){
        dates.push(
            dateTemplate + index
        )
    }
    
    
    const grapData = () => {
        if (props.data.length > 0){
            const day = Number(props.data[0].date.split('-')[2])
            if (day > 1){
                for (let index = 1; index < day; index++){
                    volumes.push(0);
                    masses.push(0);
                }
            }
            // TODO: describe case when we don't have record with daily difference in the middle of month and in the end
            // TODO: if between dates exist spaces with no data we should set zeros for such dates
            for (let index = 0; index < props.data.length; index++){            
                volumes.push(props.data[index].volume);
                masses.push(props.data[index].mass);
            }
            volumes.push(0);
            masses.push(0);
        } else {
            for (let value = 0; value < dates.length; value++){
                volumes.push(0)
                masses.push(0)
            }
        }
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