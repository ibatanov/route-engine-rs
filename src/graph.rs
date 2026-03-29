use crate::errors::GraphError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct NodeId(usize);

impl NodeId {
    /// Создает идентификатор вершины из индекса.
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    /// Возвращает внутренний индекс вершины.
    pub fn index(self) -> usize {
        self.0
    }
}

impl From<usize> for NodeId {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

impl From<NodeId> for usize {
    fn from(value: NodeId) -> Self {
        value.index()
    }
}

pub struct Graph<N, E> {
    nodes: Vec<N>,
    adj: Vec<Vec<(NodeId, E)>>,
    edge_count: usize,
}

impl<N, E> Graph<N, E> {
    /// Создает пустой граф без вершин и ребер.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            adj: Vec::new(),
            edge_count: 0,
        }
    }

    /// Добавляет вершину и возвращает ее идентификатор.
    pub fn add_node(&mut self, node: N) -> NodeId {
        let id = NodeId::new(self.nodes.len());
        self.nodes.push(node);
        self.adj.push(Vec::new());
        id
    }

    /// Добавляет ориентированное ребро `from -> to`.
    ///
    /// Возвращает ошибку, если `NodeId` выходит за границы или нарушен инвариант графа.
    pub fn add_edge(&mut self, from: NodeId, to: NodeId, edge: E) -> Result<(), GraphError> {
        let node_count = self.nodes.len();
        if self.adj.len() != node_count {
            return Err(GraphError::GraphInvariantBroken {
                node_count,
                adjacency_count: self.adj.len(),
            });
        }
        if from.index() >= node_count {
            return Err(GraphError::FromNodeOutOfBounds {
                index: from.index(),
                node_count,
            });
        }
        if to.index() >= node_count {
            return Err(GraphError::ToNodeOutOfBounds {
                index: to.index(),
                node_count,
            });
        }

        self.adj[from.index()].push((to, edge));
        self.edge_count += 1;
        Ok(())
    }

    /// Возвращает текущее количество вершин.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Возвращает текущее количество ребер.
    pub fn edge_count(&self) -> usize {
        self.edge_count
    }

    /// Возвращает `true`, если граф не содержит вершин.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Возвращает срез всех вершин.
    pub fn nodes(&self) -> &[N] {
        self.nodes.as_slice()
    }

    /// Возвращает ссылку на вершину по `NodeId`, если она существует.
    pub fn node(&self, id: NodeId) -> Option<&N> {
        self.nodes.get(id.index())
    }

    /// Возвращает изменяемую ссылку на вершину по `NodeId`, если она существует.
    pub fn node_mut(&mut self, id: NodeId) -> Option<&mut N> {
        self.nodes.get_mut(id.index())
    }

    /// Возвращает список исходящих ребер вершины, если `NodeId` корректный.
    pub fn neighbors(&self, id: NodeId) -> Option<&[(NodeId, E)]> {
        self.adj.get(id.index()).map(Vec::as_slice)
    }

    /// Возвращает изменяемый список исходящих ребер вершины, если `NodeId` корректный.
    pub fn neighbors_mut(&mut self, id: NodeId) -> Option<&mut Vec<(NodeId, E)>> {
        self.adj.get_mut(id.index())
    }
}

impl<N, E> Default for Graph<N, E> {
    fn default() -> Self {
        Self::new()
    }
}
