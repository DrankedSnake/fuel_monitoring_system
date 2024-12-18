import { Chart, Colors, Legend, Title, Tooltip } from "chart.js"
import { Line, PolarArea } from "solid-chartjs"
import { For, Show, createSignal, onMount } from "solid-js"
import { createStore } from "solid-js/store";


type DailyDifference = {
    id: String,
    volume: number,
    mass: number,
    date: string,
}
type chartProps = {
    data: Array<DailyDifference>,
    date: string,
};

export default function DailyDifferencesChart(props: chartProps){
    onMount(() => {
        Chart.register(Title, Tooltip, Legend, Colors)
    })
    const [date, setDate] = createSignal(props.date ? new Date(props.date) : new Date())
    const [lastDay, setLastDay] = createSignal(new Date(date().getFullYear(), date().getMonth() + 1, 0));
    const [monthNumber, setMonthNumber] = createSignal(date().getMonth() + 1);
    const [monthString, setMonthString] = createSignal(monthNumber() >= 10 ? `${monthNumber()}` : `0${monthNumber()}`)
    const [dateTemplate, setDateTemplate] = createSignal(`${date().getFullYear()}-${monthString()}-`)
    let tmp_dates = []

    for (let index=1; index <= lastDay().getDate(); index++){
        if (index < 10){
            tmp_dates.push(dateTemplate() + `0${index}`)
        }else{
            tmp_dates.push(dateTemplate() + index)
        }
        
    }
    
    const [dates, setDates] = createStore(tmp_dates);
    const [hfoMasses, setHfoMasses] = createStore([]);
    const [hfoVolumes, setHfoVolumes] = createStore([]);
    const [mgoMasses, setMgoMasses] = createStore([]);
    const [mgoVolumes, setMgoVolumes] = createStore([]);
    const [differences, setDifferences] = createStore({});

    
    const grepData = () => {
        let difs = {}
        let tmp2_dates = []
        
        setDate(props.date ? new Date(props.date) : new Date());
        setLastDay(new Date(date().getFullYear(), date().getMonth() + 1, 0));
        setMonthNumber(date().getMonth() + 1);
        setMonthString(monthNumber() >= 10 ? `${monthNumber()}` : `0${monthNumber()}`);
        setDateTemplate(`${date().getFullYear()}-${monthString()}-`);

        for (let index=1; index <= lastDay().getDate(); index++){
            if (index < 10){
                tmp2_dates.push(dateTemplate() + `0${index}`)
            }else{
                tmp2_dates.push(dateTemplate() + index)
            }
        }

        setDates(tmp2_dates);

        dates.forEach(
            date => {
                difs[date] = {}
                difs[date]["hfo_volume"] = 0
                difs[date]["hfo_mass"] = 0
                difs[date]["mgo_volume"] = 0
                difs[date]["mgo_mass"] = 0
            }
        );
        props.data.forEach(
            difference => {
                difs[difference.date]["hfo_volume"] = difference.hfo_volume
                difs[difference.date]["hfo_mass"] = difference.hfo_mass
                difs[difference.date]["mgo_volume"] = difference.mgo_volume
                difs[difference.date]["mgo_mass"] = difference.mgo_mass
            }
        );
        setDifferences(difs);
        
        let hfovs = [];
        let hfoms = [];
        let mgovs = [];
        let mgoms = [];

        dates.forEach(
            date => {
                hfovs.push(differences[date]["hfo_volume"])
                hfoms.push(differences[date]["hfo_mass"])
                mgovs.push(differences[date]["mgo_volume"])
                mgoms.push(differences[date]["mgo_mass"])
            }
        );
        
        setHfoVolumes(hfovs);
        setHfoMasses(hfoms);
        setMgoVolumes(mgovs);
        setMgoMasses(mgoms);

        setData({
            labels: dates,
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
        })
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
    let [data, setData] = createSignal({
            labels: dates,
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
        })
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
                {grepData()}
                <Line data={data()} options={chartOptions} width={500} height={500} />
            </Show>
        </div>
    )
}