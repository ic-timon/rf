//! # page
//!
//! page 模块 - 分页工具
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! 分页工具模块
//!
//! 本模块提供了分页相关的数据结构和计算功能。
//! 用于处理数据库查询、API 响应等场景中的分页逻辑。

/// 分页信息结构体
///
/// 用于存储和计算分页相关的所有信息，包括当前页码、每页大小和总记录数。
///
/// # 字段说明
/// - `page`: 当前页码（从 1 开始）
/// - `size`: 每页显示的记录数
/// - `total`: 总记录数
#[derive(Debug, Clone)]
pub struct Page {
    pub page: usize,
    pub size: usize,
    pub total: usize,
}

impl Page {
    /// 创建新的分页信息
    ///
    /// # 参数
    /// - `page`: 当前页码（从 1 开始）
    /// - `size`: 每页显示的记录数
    /// - `total`: 总记录数
    ///
    /// # 返回值
    /// 返回一个新的 Page 实例
    ///
    /// # 示例
    /// ```ignore
    /// use rf_util::page::Page;
    ///
    /// let page = Page::new(1, 10, 100);
    /// assert_eq!(page.page, 1);
    /// assert_eq!(page.size, 10);
    /// assert_eq!(page.total, 100);
    /// ```
    pub fn new(page: usize, size: usize, total: usize) -> Self {
        Self { page, size, total }
    }

    /// 计算当前页的偏移量
    ///
    /// 偏移量用于数据库查询时跳过前面的记录。
    /// 例如：第 1 页的偏移量为 0，第 2 页的偏移量为 size。
    ///
    /// # 返回值
    /// 返回当前页的偏移量（从 0 开始）
    ///
    /// # 示例
    /// ```ignore
    /// use rf_util::page::Page;
    ///
    /// let page = Page::new(1, 10, 100);
    /// assert_eq!(page.offset(), 0);  // 第 1 页偏移量为 0
    ///
    /// let page2 = Page::new(2, 10, 100);
    /// assert_eq!(page2.offset(), 10); // 第 2 页偏移量为 10
    /// ```
    pub fn offset(&self) -> usize {
        (self.page - 1) * self.size
    }

    /// 计算总页数
    ///
    /// 根据总记录数和每页大小自动计算总页数。
    /// 如果总记录数不能被每页大小整除，会向上取整。
    ///
    /// # 返回值
    /// 返回总页数
    ///
    /// # 示例
    /// ```ignore
    /// use rf_util::page::Page;
    ///
    /// let page = Page::new(1, 10, 100);
    /// assert_eq!(page.total_pages(), 10); // 100 条记录，每页 10 条，共 10 页
    ///
    /// let page2 = Page::new(1, 10, 95);
    /// assert_eq!(page2.total_pages(), 10); // 95 条记录，每页 10 条，共 10 页
    /// ```
    pub fn total_pages(&self) -> usize {
        self.total.div_ceil(self.size)
    }

    /// 检查是否有下一页
    ///
    /// # 返回值
    /// - `true`: 存在下一页
    /// - `false`: 当前是最后一页
    ///
    /// # 示例
    /// ```ignore
    /// use rf_util::page::Page;
    ///
    /// let page = Page::new(1, 10, 100);
    /// assert_eq!(page.has_next(), true);  // 第 1 页有下一页
    ///
    /// let page2 = Page::new(10, 10, 100);
    /// assert_eq!(page2.has_next(), false); // 第 10 页是最后一页
    /// ```
    pub fn has_next(&self) -> bool {
        self.page < self.total_pages()
    }

    /// 检查是否有上一页
    ///
    /// # 返回值
    /// - `true`: 存在上一页
    /// - `false`: 当前是第一页
    ///
    /// # 示例
    /// ```ignore
    /// use rf_util::page::Page;
    ///
    /// let page = Page::new(1, 10, 100);
    /// assert_eq!(page.has_prev(), false); // 第 1 页没有上一页
    ///
    /// let page2 = Page::new(2, 10, 100);
    /// assert_eq!(page2.has_prev(), true);  // 第 2 页有上一页
    /// ```
    pub fn has_prev(&self) -> bool {
        self.page > 1
    }
}

