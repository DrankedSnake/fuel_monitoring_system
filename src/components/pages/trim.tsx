import { Show, createResource, createSignal } from "solid-js"
import Title from "../title"
import Table from "../table/table"
import AddRecordModal from "../addRecordModal"
import UploadFileModal from "../uploadFileModal"


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
            <AddRecordModal buttonText="Add trim" title="Add trim">
                <div>
                    <label for="value">Value</label>
                    <input type="text" name="value" id="value" />
                </div>
            </AddRecordModal>
            <UploadFileModal buttonText="Upload from csv file" title="Upload trims">
                <input type="file" id="trimFile" name="filename"/>
            </UploadFileModal>
        </div>
    )
}