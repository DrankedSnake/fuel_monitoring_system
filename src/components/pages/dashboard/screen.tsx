import { createResource, createSignal } from "solid-js";
import Title from "../../title";
import Table from "../../table/table";
import DropDownMenu from "../../dropDownMenu/dropDownMenu";
import { invoke } from "@tauri-apps/api/core";
import { NavigationItems } from "../../../data";
import { DailyDifferencesChart } from ".";
import SearchForm from "../../searchForm/searchForm";
import { createStore } from "solid-js/store";


export default function Dashboard() {
    const getVessels = async () => {
        return await invoke("get_vessels");
    };

    const [form, setForm] = createStore({
        vessel_id: "",
        date: new Date().toISOString().split("T")[0], // Default to today's date
    });

    const [queryParams, setQueryParams] = createSignal({ vesselId: "", date: form.date });
    const [differences, { refetch }] = createResource(queryParams, async ({ vesselId, date }) => {
        if (vesselId && date) {
            const result = await invoke("get_daily_differences_for_current_month", { vesselId, date });
            console.log("result", result);
            return result;
        }
    });

    const [vessels] = createResource(getVessels);

    const handleChangeVesselId = (id: string) => {
        setForm("vessel_id", id);
        setQueryParams({ vesselId: id, date: form.date });
    };

    const updateFormField = (fieldName: string) => (event: Event) => {
        const inputElement = event.currentTarget as HTMLInputElement;
        setForm(fieldName, inputElement.value);
        if (fieldName === "date") {
            setQueryParams({ vesselId: form.vessel_id, date: inputElement.value });
        }
        setTimeout(() => inputElement.blur(), 0);
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
                    },
                ]}
                formChangeCallback={updateFormField}
                submitCallback={submitSearchForm}
            />
            <Title value="Dashboard" />
            <DailyDifferencesChart data={Array.isArray(differences()) ? differences() : []} date={form.date} />
            <Table
                records={differences()}
                headers={NavigationItems().filter((item) => item.name === "dashboard")[0].item.tableHeaders}
            />
        </div>
    );
}