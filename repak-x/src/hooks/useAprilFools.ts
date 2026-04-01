import { useMemo } from 'react';

export function useAprilFools(): boolean {
  return useMemo(() => {
    const now = new Date();
    return now.getMonth() === 3 && now.getDate() === 1; // April = month 3 (0-indexed)
  }, []);
}
