import { createResource, createSignal } from "solid-js";
import Title from "../../title";
import Table from "../../table/table";
import DropDownMenu from "../../dropDownMenu/dropDownMenu";
import { invoke } from "@tauri-apps/api/core";
import { NavigationItems } from "../../../data";
import { DailyDifferencesChart } from ".";
import SearchForm from "../../searchForm/searchForm";
import { createStore } from "solid-js/store";


export default function Dashboard(){
    const getDifferences = async (vesselId: string) => {
        if (vesselId){
            return await invoke("get_daily_differences_for_current_month", {"vesselId":vesselId, date: form.date} )
        }
    };
    const getVessels = async () => {
        return await invoke("get_vessels");
    };
    const [activeVesselId, setActiveVesselId] = createSignal("");
    const [differences, {refetch}] = createResource(activeVesselId, getDifferences)
    const [vessels] = createResource(getVessels);
    const handleChangeVesselId = (id: string) => {
        setActiveVesselId(id);
    };
    const [form, setForm] = createStore(
        {
            vessel_id: "",
            date: "",
        }
    );
    const updateFormField = (fieldName: string) => (event: Event) => {
        const inputElement = event.currentTarget as HTMLInputElement;
        setForm({
            [fieldName]: inputElement.value
        });
    };
    const submitSearchForm = () => {
        refetch();
    };

    return (
        <div class="screen-container">
            <DropDownMenu 
                items={vessels()}
                displayValueKey="name"
                identifyValueKey="id"
                setSignalCallback={handleChangeVesselId}
                placeholder="Select vessel..."
            />
            <SearchForm
                fields={[
                    {
                        type: "date", 
                        name: "date", 
                        id: "date",
                        min: "2023-09-1",
                    }
                ]}
                formChangeCallback={updateFormField}
                submitCallback={submitSearchForm}
            >
            </SearchForm>
            <Title value="Dashboard"/>
            <DailyDifferencesChart data={differences()} date={form.date}/>
            <Table 
                records={differences()} 
                headers={NavigationItems().filter(
                    item => item.name === "dashboard"
                )[0].item.tableHeaders}
            />
        </div>
    )
};