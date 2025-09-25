import pandas as pd

def print_info(df):
    '''
    查看DataFrame对象信息
    '''
    # print(type(df))  # <class 'pandas.core.frame.DataFrame'>
    # print(df.info())
    # print(df.shape)  # (10, 4), 行数，列数
    print(df.columns)  # Index(['a', 'b', 'c', 'd'], dtype='object')
    # print(df.dtypes)  # 每列的数据类型
    # print(df.head())  # 默认输出前5行
    # print(df.head(1))  # 指定输出x行

def get_val(df):
    # print(df['a'])
    # a_df = df['a'] # a_df = df.a
    # print(a_df.head())
    # print(a_df.tail())

    # 查看多列
    # sub_df = df[['a', 'b', 'c']]
    # print(sub_df.head())

    # 查看行
    # sub_df = df.loc[0] # 查看第一行
    # print(sub_df)
    sub_df = df.loc[df.shape[0] - 1] # 查看最后一行
    print(sub_df)

if __name__ == '__main__':
    df = pd.read_csv(".//gapminder.tsv", sep='\t')

    # 查看信息
    print_info(df)

    # 获取值
    get_val(df)