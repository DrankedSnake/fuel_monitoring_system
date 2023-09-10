export type PaginationData = {
    page: number,
    per_page: number,
    total_amount: number,
}

export type PaginationProps = {
    data: PaginationData
    submitFormCallback: Func
};

export type TableProps = {
    pagination: PaginationData
    records: Array<Object>,
    headers: Array<string>,
};
