import "./table.css";

import { For } from "solid-js";
import Record from "./record";


export default function Table(props: any){
    return (
        <table class="grid-container">
            <thead class="table-header">
                <For each={Object.keys(props.records[0])}>
                    {
                        (key)=>(
                            <th>{key.replace("_", " ").toUpperCase()}</th>
                        )
                    }
                </For>
            </thead>
            <tbody>
            <For each={props.records}>
                {
                    (record)=>(
                        <Record value={record}/>
                    )
                }
            </For>
            </tbody>
        </table>
    )
}