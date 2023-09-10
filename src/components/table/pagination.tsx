import { For, createSignal } from "solid-js";
import "./pagination.css"
import { PaginationProps } from "../../types";
import { createStore } from "solid-js/store";


export default function Pagination(props: PaginationProps){
    const handlePrevPage = () => {
        if (props.data.page > 1){
            props.mutateSignal({pagination: {
                ...props.data, page: props.data.page - 1
            }})
            props.submitFormCallback();
        }
    };
    const handleNextPage = () => {
        if (props.data.page < pages[pages.length - 1]) {
            props.mutateSignal({pagination: {
                ...props.data, page: props.data.page + 1
            }})
            props.submitFormCallback();
        }
        
    };
    const handleConcreatePage = () => {};
    const [pages, setPages] = createStore([1])

    const calculatePages = () => {
        let pagesCount = Math.ceil(props.data.total_amount / props.data.per_page);
        let numbers = []
        
        if (pagesCount <= 5) {
            for (let page = 1; page <= pagesCount; page++) {
                numbers.push(page)
            }
            setPages(numbers)
        } else {
            for (let page = 1; page <= 5; page++) {
                numbers.push(page);
            }
            numbers.push("...");
            numbers.push(pagesCount);
            setPages(numbers)
        }
    };
    

    return (
        <div id="pagination">
            {calculatePages()}
            <span name="prev" onclick={handlePrevPage}>Prev</span>
            <For each={pages}>
                {
                    (page) => (
                        <span name={`${page}`}>{page}</span>
                    )
                }
            </For>
            <span name="next" onclick={handleNextPage}>Next</span>
        </div>
    )
}