import { Show, createResource } from "solid-js"
import Title from "../title"
import Table from "../table/table"


const getTrims = async ()=> {
    return []
}

export default function Trim(){
    const [trims] = createResource(getTrims)

    return (
        <div class="screen-container">
            <Title value="Trim"/>
            <Show when={trims()} fallback={<p>Loading...</p>}>
                <Table records={trims()}/>
            </Show>
            <div class="operations-bar">
                <button>Add trim</button>
                <button>Upload trims from csv file</button>
            </div>
        </div>
    )
}