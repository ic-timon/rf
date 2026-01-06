//! # tree 模块
//!
//! tree 模块 - 树容器
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # 树容器
//!
//! 提供通用的树结构容器 (`TreeContainer`)，基于 `id_tree` crate 实现。
//! 支持创建带有根节点的树，并可以动态添加子节点。
//!
//! # 特性
//!
//! - 通用的树结构：可以存储任意类型的数据
//! - 节点 ID 系统：每个节点都有唯一的 ID，便于引用和操作
//! - 支持动态插入子节点
//!
//! # 示例
//!
//! ```
//! use rf_container::TreeContainer;
//!
//! // 创建树，根节点值为 "root"
//! let mut tree = TreeContainer::new("root");
//!
//! // 获取根节点 ID
//! let root_id = tree.root_id().unwrap();
//!
//! // 插入子节点
//! let child1 = tree.insert_child(root_id, "child1").unwrap();
//! let child2 = tree.insert_child(root_id, "child2").unwrap();
//! ```

use id_tree::{Node, NodeId, Tree, TreeBuilder};

/// 树容器包装器
///
/// 封装了 `id_tree::Tree`，提供简洁的 API 用于操作树结构。
/// 每个树都有一个根节点，其他节点可以作为根节点或现有节点的子节点插入。
///
/// # 字段
///
/// - `tree`: 内部的 `Tree<T>`，存储树结构
/// - `root`: 根节点的 ID，可能为 `None`
///
/// # 类型参数
///
/// * `T`: 树节点中存储的数据类型
///
/// # 示例
///
/// ```
/// use rf_container::TreeContainer;
///
/// let mut tree = TreeContainer::new("root");
/// let root_id = tree.root_id().unwrap();
/// assert_eq!(tree.root_data(), Some(&"root"));
/// ```
#[derive(Debug)]
pub struct TreeContainer<T> {
    tree: Tree<T>,
    root: Option<NodeId>,
}

impl<T> TreeContainer<T> {
    /// 创建一个新的树，并设置根节点
    ///
    /// # 参数
    ///
    /// * `root_data`: 根节点中存储的数据
    ///
    /// # 返回值
    ///
    /// 返回一个带有根节点的 `TreeContainer` 实例
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::TreeContainer;
    ///
    /// let tree = TreeContainer::new("Root");
    /// ```
    pub fn new(root_data: T) -> Self {
        let mut tree = TreeBuilder::new().build();
        let root = tree.insert(Node::new(root_data), id_tree::InsertBehavior::AsRoot).ok();
        Self { tree, root }
    }

    /// 获取根节点的 ID
    ///
    /// # 返回值
    ///
    /// 返回根节点的 ID 的引用，如果树为空则返回 `None`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::TreeContainer;
    ///
    /// let tree = TreeContainer::new("root");
    /// let root_id = tree.root_id();
    /// assert!(root_id.is_some());
    /// ```
    pub fn root_id(&self) -> Option<&NodeId> {
        self.root.as_ref()
    }

    /// 获取根节点中存储的数据
    ///
    /// # 返回值
    ///
    /// 返回根节点数据的引用，如果树为空则返回 `None`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::TreeContainer;
    ///
    /// let tree = TreeContainer::new("root data");
    /// assert_eq!(tree.root_data(), Some(&"root data"));
    /// ```
    pub fn root_data(&self) -> Option<&T> {
        self.root.as_ref().and_then(|id| {
            self.tree.get(id).ok().map(|n| n.data())
        })
    }

    /// 在指定父节点下插入一个子节点
    ///
    /// # 参数
    ///
    /// * `parent_id`: 父节点的 ID
    /// * `data`: 要在子节点中存储的数据
    ///
    /// # 返回值
    ///
    /// - 成功时返回 `Ok(NodeId)`，即新创建的子节点的 ID
    /// - 失败时返回 `Err(id_tree::NodeIdError)`，通常表示父节点 ID 无效
    ///
    /// # 错误
    ///
    /// - 如果 `parent_id` 不存在，会返回错误
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::TreeContainer;
    ///
    /// let mut tree = TreeContainer::new("root");
    /// let root_id = tree.root_id().unwrap();
    ///
    /// // 插入子节点
    /// let child_id = tree.insert_child(root_id, "child").unwrap();
    /// ```
    pub fn insert_child(&mut self, parent_id: &NodeId, data: T) -> Result<NodeId, id_tree::NodeIdError> {
        self.tree.insert(Node::new(data), id_tree::InsertBehavior::UnderNode(parent_id))
    }
}
