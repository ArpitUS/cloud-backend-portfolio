def transform(df):
    df.columns = [col.lower() for col in df.columns]
    return df.dropna()
