import { Show, createResource } from "solid-js";
import Title from "../title";
import Table from "../table/table";


const getDifferences = async ()=> {
    return []
};

export default function Differences(){
    const [differences] = createResource(getDifferences)
    
    return (
        <div class="screen-container">
            <Title value="Differences"/>
            <Show when={differences()} fallback={<p>Loading...</p>}>
                <Table records={differences()}/>
            </Show>
            <div class="operations-bar">
                <button>Add difference</button>
            </div>
        </div>
    )
}