export function formatMoney(cents: number): string {
  const sign = cents < 0 ? "-" : "";
  const abs = Math.abs(cents);
  return sign + "¥" + (abs / 100).toFixed(2);
}
export function centsToYuan(cents: number): number {
  return cents / 100;
}
export function yuanToCents(yuan: number): number {
  return Math.round(yuan * 100);
}
export function formatDate(iso: string): string {
  if (!iso) return "";
  return iso.slice(0, 10);
}
