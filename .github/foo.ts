export type SimpleGraph<T> = Map<T, T[]>;

export const breadthFirstSearch = <T>(graph: SimpleGraph<T>, start: T): T[] => {
  let bfsOrder: T[] = [];

  const bfs = (queue: T[], visited: Set<T>) => {
    if (queue.length === 0) return;

    const [node, ...rest] = queue;
    bfsOrder = [...bfsOrder, node];

    const unvisitedNeighbors =
      graph.get(node)?.filter((neighbor) => !visited.has(neighbor)) || [];
    const newQueue = [...rest, ...unvisitedNeighbors];
    const newVisited = new Set([...visited, ...unvisitedNeighbors]);

    bfs(newQueue, newVisited);
  };

  bfs([start], new Set([start]));

  return bfsOrder;
};
