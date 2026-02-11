export function parseMarket(raw: any) {
  return {
    question: raw.question,
    resolved: raw.resolved,
  };
}
