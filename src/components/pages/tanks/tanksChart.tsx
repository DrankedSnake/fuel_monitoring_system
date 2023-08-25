import { Bar } from "solid-chartjs"
import { Tank } from "../../../types"
import { Show } from "solid-js";
import { Chart, Legend, Tooltip } from "chart.js";


type TanksChartProps = {
    tanks: Array<Tank>
}

export default function TanksChart(props: TanksChartProps){
    let labels = [];
    let full_volumes = [];
    let current_volumes = [];
    let safe_volumes = [];
    let change_24_volumes = [];
    let bunkering_volumes = [];

    const prepareDataset = () => {
        for (let index = 0; index < props.tanks.length; index++){
            labels.push(props.tanks[index].name)
            full_volumes.push(props.tanks[index].full_volume)
            current_volumes.push(props.tanks[index].current_volume)
            safe_volumes.push(props.tanks[index].safe_volume)
            change_24_volumes.push(props.tanks[index].change_24_volume)
            bunkering_volumes.push(props.tanks[index].bunkering_volume)

        }
        console.log(change_24_volumes)
    };

    let data = {
        labels: labels,
        datasets: [
            // {
            //     label: "Full volume",
            //     data: full_volumes,
            //     backgroundColor: "rgba(255, 99, 132, 0.2)",
            //     borderColor: "rgb(255, 99, 132)",
            //     borderWidth: 1
            // },
            // {
            //     label: "Safe volume",
            //     data: safe_volumes,
            //     backgroundColor: "rgba(255, 159, 64, 0.2)",
            //     borderColor: "rgb(255, 159, 64)",
            //     borderWidth: 1
            // },


            {
                label: "Current volume",
                data: current_volumes,
                backgroundColor: "rgba(255, 205, 86, 0.5)",
                borderColor: "rgb(255, 205, 86)",
                borderWidth: 1
            },
            // {
            //     label: "Change 24 hours volume",
            //     data: change_24_volumes,
            //     backgroundColor: "rgba(255, 100, 86, 0.2)",
            //     borderColor: "rgb(255, 205, 86)",
            //     borderWidth: 1
            // },
            {
                label: "Bunkering volume",
                data: bunkering_volumes,
                backgroundColor: "rgba(154, 205, 86, 0.5)",
                borderColor: "rgb(255, 205, 86)",
                borderWidth: 1
            },
        ]
    }
    const mockedData = {
        labels: [],
        datasets: []
    };
    const chartOptions = {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
            legend: {
                display: true,
            }
        },
        scales: {
            x: {
                stacked: true,
            },
            y: {
                stacked: true
            }
        }
    }
    Chart.register(
        Legend,
    )
    Chart.register(
        Tooltip,
    )
    return (
        <div>
            <Show 
                when={props.tanks}
                fallback={
                    <Bar data={mockedData} options={chartOptions} width={500} height={500}/>
                }
            >
                {prepareDataset()}
                <Bar data={data} options={chartOptions} width={500} height={500}/>
            </Show>

        </div>
    )
}