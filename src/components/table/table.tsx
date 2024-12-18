import "./table.css";

import { For, Show } from "solid-js";
import Record from "./record";
import Pagination from "./pagination";
import { TableProps } from "../../types";


export default function Table(props: TableProps){
        return (
            <div class="board">
            <table>
                <thead class="table-header">
                    <tr>
                        <For each={props.headers}>
                            {
                                (header: string)=>{
                                    return (
                                        <td>
                                            {header.replace("_", " ")}
                                        </td>
                                    )                            
                                }
                            }
                        </For>
                    </tr>
                </thead>
                <tbody>
                    <Show when={props.records}>
                        <For each={props.records} fallback={<p>Loading...</p>}>
                        {
                            (record)=>(
                                <Record record={record} headers={props.headers}/>
                            )
                        }
                        </For>
                    </Show>
                </tbody>
            </table>
            <Show when={props.pagination}>
                <Pagination
                    data={props.pagination}
                    submitFormCallback={props.submitFormCallback}
                    mutateSignal={props.mutateSignal}
                />
            </Show>
            
            </div>
        )
}