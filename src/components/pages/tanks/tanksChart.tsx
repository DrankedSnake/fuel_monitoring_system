import { Bar } from "solid-chartjs"
import { Tank } from "../../../types"
import { Show, createResource, onMount, OnEffectFunction, createSignal } from "solid-js";
import { Chart, Legend, Tooltip } from "chart.js";


type TanksChartProps = {
    tanks: Array<Tank>
}

export default function TanksChart(props: TanksChartProps){
    onMount(()=>{Chart.register(Legend, Tooltip)})
    const [data, setData] = createSignal(
        {
            labels: [],
            datasets: []
        }
    )

    const prepareDataset = () => {
        if (props.tanks){
            let labels = [];
            let currentVolumes = [];
            let bunkeringVolumes = [];

            for (let index = 0; index < props.tanks.length; index++){
                labels.push(props.tanks[index].name)
                currentVolumes.push(props.tanks[index].current_volume)
                bunkeringVolumes.push(props.tanks[index].bunkering_volume)
            }
            setData(
                {
                    labels: labels,
                    datasets: [
                        {
                            label: "Current volume",
                            data: currentVolumes,
                            backgroundColor: "rgba(255, 205, 86, 0.5)",
                            borderColor: "rgb(255, 205, 86)",
                            borderWidth: 1
                        },
                        {
                            label: "Bunkering volume",
                            data: bunkeringVolumes,
                            backgroundColor: "rgba(154, 205, 86, 0.5)",
                            borderColor: "rgb(255, 205, 86)",
                            borderWidth: 1
                        },
                    ]
                }
            )
        }else {
            setData(
                {
                    labels: [],
                    datasets: []
                }
            );
        }

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
    
    return (
        <div class="chart">
            <Show 
                when={props.tanks}
                fallback={
                    <Bar data={data()} options={chartOptions} width={500} height={500}/>
                }
            >
                {prepareDataset()}
                <Bar data={data()} fallback={console.log("rerender of chart")} options={chartOptions} width={500} height={500}/>
            </Show>
        </div>
    )
}