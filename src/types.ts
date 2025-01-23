type Item {
  id: number,
  name: string,
  location: string,
  min?: number,
  max?: number,
  current: number,
  supplier?: number,
  updated: number,
  link?: string,
}

type AddItem {
  name: string,
  location: string,
  min?: number,
  max?: number,
  current: number,
  supplier?: number,
  link?: string,
}

type UpdateItem {
  id: number,
  name: string,
  location: string,
  min?: number,
  max?: number,
  current: number,
  supplier?: number,
  link?: string,
}

type Supplier {
  name: string,
  link?: string,
  notes?: string,
  username?: string,
  password?: string,
  club: string,
}

type ShortageItem {
  name: string,
  location: string,
  min: number,
  current_amount: number,
  order_amount: number,
}

type Log {
  amount: number,
  time: number,
}
