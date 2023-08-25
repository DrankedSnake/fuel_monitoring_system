import "./paggination.css"

type PaginationData = {

}

type PaginationProps = {
    data: PaginationData
};

export default function Pagination(props){
    return (
        <div id="pagination">
            <span>Prev</span>
            <span>Next</span>
        </div>
    )
}