import "./table.css";

import { For } from "solid-js";
import Record from "./record";


export default function Table(props: any){
    if (props.records) {
        return (
            <table class="grid-container">
                <thead class="table-header">
                    <For each={props.headers}>
                        {
                            (header)=>{
                                return (
                                    <th>{header}</th>
                                )                            
                            }
                        }
                    </For>
                </thead>
                <tbody>
                <For each={props.records}>
                {
                    (record)=>(
                        <Record record={record} headers={props.headers}/>
                    )
                }
                </For>
                </tbody>
            </table>
        )
    }
    else {
        return (<p>No data</p>)
    }
}