import { Chart, Colors, Legend, Title, Tooltip } from "chart.js";
import { Line } from "solid-chartjs";
import { Show, createSignal, onMount, createEffect } from "solid-js";

type DailyDifference = {
    id: String;
    volume: number;
    mass: number;
    date: string;
};
type chartProps = {
    data: Array<DailyDifference>;
    date: string;
};

export default function DailyDifferencesChart(props: chartProps) {
    onMount(() => {
        Chart.register(Title, Tooltip, Legend, Colors);
    });

    const [data, setData] = createSignal({
        labels: [],
        datasets: [],
    });

    const chartOptions = {
        responsive: true,
        maintainAspectRatio: false,
    };
    const mock_date = props.date ? new Date(props.date) : new Date();
    const mock_lastDay = new Date(mock_date.getFullYear(), mock_date.getMonth() + 1, 0);
    const mock_monthNumber = mock_date.getMonth() + 1;
    const mock_monthString = mock_monthNumber >= 10 ? `${mock_monthNumber}` : `0${mock_monthNumber}`;
    const mock_dateTemplate = `${mock_date.getFullYear()}-${mock_monthString}-`;

    const mock_dates = [];
    for (let index = 1; index <= mock_lastDay.getDate(); index++) {
        mock_dates.push(index < 10 ? `${mock_dateTemplate}0${index}` : `${mock_dateTemplate}${index}`);
    }

    createEffect(() => {
        // Ensure props.data is defined and properly structured
        if (!props.data || !Array.isArray(props.data)) {
            console.error("Invalid props.data:", props.data);
            return;
        }
    
        console.log("props.data", props.data);
    
        const date = props.date ? new Date(props.date) : new Date();
        const lastDay = new Date(date.getFullYear(), date.getMonth() + 1, 0);
        const monthNumber = date.getMonth() + 1;
        const monthString = monthNumber >= 10 ? `${monthNumber}` : `0${monthNumber}`;
        const dateTemplate = `${date.getFullYear()}-${monthString}-`;
    
        const tmp_dates = [];
        for (let index = 1; index <= lastDay.getDate(); index++) {
            tmp_dates.push(index < 10 ? `${dateTemplate}0${index}` : `${dateTemplate}${index}`);
        }
    
        const differences = {};
        tmp_dates.forEach((date) => {
            differences[date] = {
                hfo_volume: 0,
                hfo_mass: 0,
                mgo_volume: 0,
                mgo_mass: 0,
            };
        });
    
    
        props.data.forEach((difference) => {
            if (
                difference.date &&
                differences[difference.date] &&
                typeof difference.hfo_volume === "number" &&
                typeof difference.hfo_mass === "number" &&
                typeof difference.mgo_volume === "number" &&
                typeof difference.mgo_mass === "number"
            ) {
                differences[difference.date] = {
                    hfo_volume: difference.hfo_volume,
                    hfo_mass: difference.hfo_mass,
                    mgo_volume: difference.mgo_volume,
                    mgo_mass: difference.mgo_mass,
                };
            } else {
                console.warn("Invalid difference entry:", difference);
            }
        });
    
        const hfoVolumes = tmp_dates.map((date) => differences[date].hfo_volume);
        const hfoMasses = tmp_dates.map((date) => differences[date].hfo_mass);
        const mgoVolumes = tmp_dates.map((date) => differences[date].mgo_volume);
        const mgoMasses = tmp_dates.map((date) => differences[date].mgo_mass);
    
        setData({
            labels: tmp_dates,
            datasets: [
                {
                    label: `HFO Volume`,
                    data: hfoVolumes,
                    pointStyle: "circle",
                    pointRadius: 10,
                },
                {
                    label: `HFO Mass`,
                    data: hfoMasses,
                    pointStyle: "triangle",
                    pointRadius: 10,
                },
                {
                    label: `MGO Volume`,
                    data: mgoVolumes,
                    pointStyle: "circle",
                    pointRadius: 10,
                },
                {
                    label: `MGO Mass`,
                    data: mgoMasses,
                    pointStyle: "triangle",
                    pointRadius: 10,
                },
            ],
        });
    });
    const mockData = {
                labels: mock_dates,
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
    return (
        <div class="chart">
            <Show
                when={props.data && props.data.length > 0}
                fallback={<Line data={mockData} options={chartOptions} width={500} height={500} />}
            >
                <Line data={data()} options={chartOptions} width={500} height={500} />
            </Show>
        </div>
    );
}