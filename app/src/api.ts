type EntryItem = {
    BulkString: string,
    Array: EntryItem[];
};
type Entries = [EntryItem];

export async function getEntries(origin: string): Promise<Entries> {
    const url = origin + "/api/entries";
    const res = await fetch(url);
    const result = await res.json() as Entries;
    return result;
}
