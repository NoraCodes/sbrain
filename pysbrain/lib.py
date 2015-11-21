# pysbrain
# Implements Semantic Brain transliteration and execution


class SBrainVirtualMachine:
    """
    An implementation of the SBrain Virtual Machine
    """
    def __init__(self):
        self.data_t = [0]  # The data tape
        self._len_data_t = 1 # The current length of the data tape

        self.data_s = [0]  # The data stack
        self._len_data_s = 0 # The current depth of the data stack

        self.exec_t = [0]  # The executable tape, containing instructions
        self._len_exec_t = 1 # The length of the executable tape; should not change

        self.jump_s = [0]  # The jump stack, containing addresses of [ (4) instructions
        self._len_jump_s = 0 # The depth of the jump stack

        self.data_p, self.inst_p, self.jump_p, self.auxi_r = 0  # Register and pointers

    def load_data_tape(self, data_tape):
        """
        Load a data tape into the virtual machine
        :param data_tape: The tape to load; a list of ints.
        :return: None
        """
        self.data_t = data_tape
        self._len_data_t = len(data_tape)
        self.data_p = 0

    def load_executable_tape(self, exec_tape):
        """
        Load an executable tape (a transcribed program) into the virtual machine
        :param exec_tape: The tape to load: a list of ints
        :return: None
        """
        self.exec_t = exec_tape
        self._len_exec_t = len(exec_tape)
        self.inst_p = 0

    def write_data_t(self, value):
        """
        Write a value to the data tape
        :param value: The value to be written
        :return: None
        """
        if type(value) is not int:
            raise ValueError("Attempt to write a non-int value to the data tape. PANIC!")
        self.data_t[self.data_p] = value

    def read_data_t(self):
        """
        Read data_t and return the value
        :return: The value at data_p on data_t, int
        """
        if self._len_data_t >= self.data_p:
            return self.data_t[self.data_p]

    def push_data_s(self, value):
        """
        Push a value onto data_s
        :param value:
        :return:
        """
        if type(value) is not int:
            raise ValueError("Attempt to push a non-int value in the virtual machine. PANIC!")
        self.data_s.append(value)
        self._len_data_s += 1

    def pop_data_s(self):
        """
        Pop one value from data_s and return it
        Returns a 0 if the stack is empty
        :return int
        """
        if self._len_data_s > 0:
            self._len_data_s -= 1
            return self.data_s.pop()
        else:
            return 0

