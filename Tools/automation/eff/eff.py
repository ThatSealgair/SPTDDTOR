import pandas as pd


def message(name, *kwargs):
     return f"Hello {name}," 
     + f"\nYou ordered {item1} from us on the {item2} and have an outstand balance of {item3}."
     + f"\nYou will need to compete this payment before we can send you your order"



if __name__ == '__main__':
    spreadsheet = "test.xlsx"
    seperator = ","

    file = pd.read_csv(f"date/{spreadsheet}", sep=seperator, skipinitialspace=True)


