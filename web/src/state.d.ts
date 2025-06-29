export interface ItemData {
  id: number;
  key: string;
  fields: object;
  score: number | null;
  matchIndices: number[] | null;
}

