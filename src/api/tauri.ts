import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export interface Category {
  id: number;
  name: string;
  type: "income" | "expense";
  sort_order: number;
  created_at: string;
}
export interface Transaction {
  id: number;
  date: string;
  amount: number;
  type: "income" | "expense";
  category_id: number | null;
  category_name?: string;
  note: string;
  created_at: string;
  updated_at: string;
  image_count?: number;
  balance?: number;
}
export interface TransactionDetail extends Transaction {
  images: { id: number; file_path: string; file_name: string; created_at: string }[];
}
export interface Stats {
  total_income: number;
  total_expense: number;
  balance: number;
  by_category: { name: string; type: string; total: number }[];
  by_month: { month: string; income: number; expense: number }[];
}

function isTauri(){ return typeof window !== "undefined" && (("__TAURI__" in window) || ("__TAURI_INTERNALS__" in window)); }

// mock for browser preview
const mockCategories: Category[] = [
  { id: 1, name: "耗材", type: "expense", sort_order: 1, created_at: "" },
  { id: 2, name: "设备", type: "expense", sort_order: 2, created_at: "" },
  { id: 3, name: "差旅", type: "expense", sort_order: 3, created_at: "" },
  { id: 4, name: "劳务", type: "expense", sort_order: 4, created_at: "" },
  { id: 5, name: "测试费", type: "expense", sort_order: 5, created_at: "" },
  { id: 6, name: "经费拨款", type: "income", sort_order: 6, created_at: "" },
];
let mockTransactions: Transaction[] = [
  { id: 1, date: "2026-08-20", amount: 125000, type: "income", category_id: 6, category_name: "经费拨款", note: "学校拨款 Q3", created_at: "", updated_at: "", image_count: 1 },
  { id: 2, date: "2026-08-21", amount: 32000, type: "expense", category_id: 1, category_name: "耗材", note: "购买离心管、枪头", created_at: "", updated_at: "", image_count: 2 },
  { id: 3, date: "2026-08-22", amount: 8500, type: "expense", category_id: 3, category_name: "差旅", note: "高铁票报销", created_at: "", updated_at: "", image_count: 0 },
];

export const api = {
  listCategories(): Promise<Category[]> {
    if (isTauri()) return invoke("list_categories");
    return Promise.resolve([...mockCategories]);
  },
  createCategory(name: string, type: "income" | "expense"): Promise<number> {
    if (isTauri()) return invoke("create_category", { name, type_ : type });
    const id = Date.now();
    mockCategories.push({ id, name, type, sort_order: 99, created_at: new Date().toISOString() });
    return Promise.resolve(id);
  },
  updateCategory(id: number, name: string): Promise<void> {
    if (isTauri()) return invoke("update_category", { id, name });
    const c = mockCategories.find(x=>x.id===id); if(c) c.name=name;
    return Promise.resolve();
  },
  deleteCategory(id: number): Promise<void> {
    if (isTauri()) return invoke("delete_category", { id });
    const idx = mockCategories.findIndex(x=>x.id===id); if(idx>=0) mockCategories.splice(idx,1);
    return Promise.resolve();
  },
  listTransactions(params: any): Promise<{items: Transaction[], total: number, balance: number}> {
    if (isTauri()) return invoke("list_transactions", params);
    let items = [...mockTransactions];
    if (params.keyword) items = items.filter(t=> t.note.includes(params.keyword));
    if (params.type_) items = items.filter(t=> t.type===params.type_);
    // compute running balance
    let bal = 0;
    const sorted = [...items].sort((a,b)=> a.date.localeCompare(b.date) || a.id-b.id);
    sorted.forEach(t=> { if(t.type==="income") bal+=t.amount; else bal-=t.amount; (t as any).balance = bal; });
    const total = items.length;
    const page = params.page || 1; const ps = params.page_size || 20;
    const paged = sorted.slice((page-1)*ps, page*ps).reverse();
    const overall = mockTransactions.reduce((s,t)=> s + (t.type==="income"? t.amount : -t.amount), 0);
    return Promise.resolve({ items: paged, total, balance: overall });
  },
  getStats(): Promise<Stats> {
    if (isTauri()) return invoke("get_stats", {});
    const inc = mockTransactions.filter(t=>t.type==="income").reduce((s,t)=>s+t.amount,0);
    const exp = mockTransactions.filter(t=>t.type==="expense").reduce((s,t)=>s+t.amount,0);
    return Promise.resolve({
      total_income: inc, total_expense: exp, balance: inc-exp,
      by_category: [
        { name: "耗材", type: "expense", total: 32000 },
        { name: "差旅", type: "expense", total: 8500 },
        { name: "经费拨款", type: "income", total: 125000 },
      ],
      by_month: [{ month: "2026-08", income: 125000, expense: 40500 }]
    });
  },
  getDashboardSummary(): Promise<{balance:number, month_income:number, month_expense:number}>{
    if(isTauri()) return invoke("get_dashboard_summary");
    return Promise.resolve({ balance: 84500, month_income: 125000, month_expense: 40500 });
  },
  createTransaction(payload: any): Promise<number> {
    if(isTauri()) return invoke("create_transaction", { payload });
    const id = Date.now();
    mockTransactions.push({ id, date: payload.date, amount: Math.round(payload.amount*100), type: payload.type_, category_id: payload.category_id, category_name: mockCategories.find(c=>c.id===payload.category_id)?.name, note: payload.note, created_at:"", updated_at:"", image_count: 0 });
    return Promise.resolve(id);
  },
  updateTransaction(id:number, payload:any): Promise<void>{
    if(isTauri()) return invoke("update_transaction", { id, payload });
    const t = mockTransactions.find(x=>x.id===id); if(t) Object.assign(t, { date:payload.date, amount:Math.round(payload.amount*100), type:payload.type_, category_id:payload.category_id, note:payload.note, category_name: mockCategories.find(c=>c.id===payload.category_id)?.name });
    return Promise.resolve();
  },
  deleteTransaction(id:number): Promise<void>{
    if(isTauri()) return invoke("delete_transaction", { id });
    mockTransactions = mockTransactions.filter(x=>x.id!==id);
    return Promise.resolve();
  },
  getTransaction(id:number): Promise<TransactionDetail>{
    if(isTauri()) return invoke("get_transaction", { id });
    const t = mockTransactions.find(x=>x.id===id)!;
    return Promise.resolve({ ...t, images: [] });
  },
  exportExcel(filter:any): Promise<string>{
    if(isTauri()) return invoke("export_excel", { filter });
    return Promise.resolve("浏览器预览模式暂不支持导出");
  }
};
