import { Show, createResource } from "solid-js";
import Title from "../title";
import Table from "../table/table";


const getTankers = async ()=>{
    return []
};

export default function Tankers(){
    const [tankers] = createResource(getTankers)
    return (
        <div class="screen-container">
            <Title value="Tankers"/>
            <Show when={tankers()} fallback={<p>Loading...</p>}>
                <Table records={tankers()}/>
            </Show>
            <div class="operations-bar">
                <button>Add tanker</button>
            </div>
        </div>
    )
}