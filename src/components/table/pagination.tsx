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
    const [currentPage, setCurrentPage] = createSignal(1)

    const pushNumbers = (startNumber: number, endNumber: number, totalNumber: number, numbers: Array<number | string>) => {
        if (totalNumber <= startNumber) {
            for (let page = startNumber; page <= totalNumber; page++) {
                numbers.push(page)
            }
        } else {
            for (let page = startNumber; page <= endNumber; page++) {
                numbers.push(page);
            }
            numbers.push("...");
            numbers.push(totalNumber);
        }
    }

    const calculatePages = () => {
        let pagesCount = Math.ceil(props.data.total_amount / props.data.per_page);
        let numbers: Array<number | string> = []
        let visiblePagesLength = 5
        let startVisiblePage = 1;
        let endVisiblePage = 5;

        if (props.data.page <= visiblePagesLength){
            pushNumbers(startVisiblePage, endVisiblePage, pagesCount, numbers)
        } else {
            startVisiblePage += 1;
            endVisiblePage += 1;
            pushNumbers(startVisiblePage, endVisiblePage, pagesCount, numbers)
        }
        setPages(numbers)
    };

    return (
        <div id="pagination">
            {calculatePages()}
            <span name="prev" onclick={handlePrevPage}>Prev</span>
            <For each={pages}>
                {
                    (page) => (
                        <span 
                            class={
                                page === props.data.page ? "red-button" : ""
                            } 
                            name={`${page}`}
                        >
                            {page}
                        </span>
                    )
                }
            </For>
            <span name="next" onclick={handleNextPage}>Next</span>
        </div>
    )
}