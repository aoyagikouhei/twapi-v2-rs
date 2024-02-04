
export interface ContextAnnotations {
  domain: Domain | undefined;
  entity: Entity | undefined;
}

export interface Domain {
  id: string | undefined;
  name: string | undefined;
  description: string | undefined;
}

export interface Entity {
  id: string | undefined;
  name: string | undefined;
  description: string | undefined;
}